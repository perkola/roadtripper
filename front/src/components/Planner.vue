<template lang="jade">
//
  map(
      :center.sync="center",
      :zoom.sync="zoom",
      :map-type-id.sync="mapType"
  )
div.navbar
    h1.navbar__logo #[a(v-link="{ path: '/' }") R#[span.hide-for-small oadtripper]]
    div.navbar__city-search(:class="{ 'test': yo }")
        span(v-show="yo", transition="expand") Type the name of a city...
        input(
            type="text", placeholder="San Francisco", v-model="city",
            @blur="clear()",
            @keyup.enter="addNewCity",
            @keyup.down="selectNext()",
            @keyup.up="selectPrev()",
            @keyup="autocomplete | debounce 200"
        )
        div.predictions
            span.prediction(
                v-for="(index, prediction) in predictions",
                :class="{ 'prediction--selected': index == predIndex }",
                @mouseenter="selectThis(index)",
                @click="addNewCity"
            )
                {{ prediction.description }}
        button(@click="addNewCity") #[i.material-icons add]
    a.navbar__button(v-if="cities.length", transition="modal", @click="saveRoadtrip()") #[span.hide-for-small Save roadtrip] #[i.material-icons exit_to_app]

div.planner

  div.planner-cities
      city(v-for="(index, city) in cities", :city="city", :class="index", transition="modal")

Timeline

div.alert
    div.alert__mask(v-show="showAlert", transition="modal")
        div.alert__wrapper
            div.alert__container
                a.alert__close(@click="showAlert = false") #[i.material-icons close]
                i.done.material-icons done
                h1.alert__heading Roadtrip saved!
                p Here's the unique link to your roadtrip. Go ahead and share it with your friends:
                span.link {{ tripLink }}
                p If they have any suggestions, feel free to edit the roadtrip, changes will be saved in a new roadtrip!

</template>

<script>

//import {load, Map } from 'vue-google-maps'
import crypto from 'crypto'
import Timeline from './Timeline.vue'
import City from './City.vue'
import { addCity, setDuration, setDate } from '../vuex/actions'
import m from 'moment'

//load('AIzaSyDB69x_sL1X3tawNIFdht4prhEW9bymssc', '3.23', ['places']);

export default {
  name: 'Planner',
  data: function data() {
    return {
      //cities: [],
      totalTime: "",
      predictions: [],
      predIndex: 0,
      center: { lat: 10, lng: 11 },
      zoom: 5,
      mapType: 'terrain',
      city: null,
      showAlert: false,
      planID: null
    }
  },
  vuex: {
      getters: {
          startdate: state => state.dates.startdate,
          enddate: state => state.dates.enddate,
          cities: state => state.cities,
          duration: state => state.duration
      },
      actions: {
          addCity,
          setDate,
          setDuration
      }
  },
  ready() {
    var idx = this.$route.params.id
    if (idx) {
        var self = this
        var data = this.$http({ url: '/api/roadtrip', method: 'GET', params: { token: idx }}).then(function (response) {
            return response.data
        }, function(response) {
            return null
        })
        data.then(function(yo) {
            var self = this
            yo.cities.forEach(function(city) {
                addCity(self.$store, city)
            })
            setDate(this.$store, 'startdate', yo.startdate)
            //console.log("1", yo.startdate)
            setDate(this.$store, 'enddate', yo.enddate)
            //console.log("2", yo.enddate)
        })
    }
  },
  computed: {
      yo() {
          return !this.city && this.cities.length < 1
      },
      tripLink() {
          return window.location.protocol + '//' + window.location.host + '/#!/plan/' + this.planID
      }
  },
  methods: {
    clear() {
        setTimeout( function(){
            //this.predIndex = 0
            //this.predictions = []
        }.bind(this),100)
    },
    selectThis(index) {
        this.predIndex = index
    },
    selectNext() {
        if (this.predIndex < 4) {
            this.predIndex++
            //this.city = this.predictions[this.predIndex].description
        }
    },
    selectPrev() {
        if (this.predIndex > 0) {
            this.predIndex--
            //this.city = this.predictions[this.predIndex].description
        }
    },
    showCity: function(e) {
      //console.log(e)
    },
    autocomplete: function(e) {
        if (! this.city) {
            this.predictions = []
            return;
        }
        var autocomplete = this.$http.post('/api/autocomplete', this.city);
        var self = this;
        autocomplete.then(function(data) {
            if (data['data']['predictions']) {
                ////console.log(data['data']['predictions']);
                self.predictions = data['data']['predictions'];
                //console.log(self.predictions[0]['description']);
            }
        });
    },
    removeCity(city) {
        //console.log(city)
    },
    addNewCity: function() {
      if (! this.city) {
          return
      }
      var self = this
      var selectedCity = this.predictions[this.predIndex].description
      var city = { id: crypto.randomBytes(20).toString('hex'), name: selectedCity.split(",")[0], activities: [], count: 1, transitionTime: '-', nextCity: '', rawObj: this.predictions[this.predIndex] }
      //console.log(this.predictions[this.predIndex]);
      addCity(this.$store, city)
      if (this.cities.length > 1) {
          var prevCity = this.cities[this.cities.length - 2];
          prevCity['nextCity'] = city;
          var res = this.getCityDistance(prevCity['rawObj'], city['rawObj']);
          res.then(function(value) {
              self.updateCityTransition(prevCity, value);
          }, function(value) {
              //console.log("Failed to get city distance");
          });
      }
      this.city = null
      this.predictions = []
      this.predIndex = 0
      //addCity(this.$store, city)
    },
    /* this will update all transition times in the roadtrip */
    updateCityTransitions() {
      //console.log("Updating city transitions");
        if (this.cities.length > 1) {
            var self = this;
            for (var i = 0; i < this.cities.length - 1; i++) {
                var thisCity = this.cities[i];
                var nextCity = this.cities[i+1];

                thisCity['nextCity'] = nextCity;
                var res = this.getCityDistance(thisCity['rawObj'], nextCity['rawObj']);
                res.then(function(value) {
                    self.updateCityTransition(thisCity, value);
                }, function (value) {
                    //console.log("Failed to get city distance between ",
                    // thisCity['name'], " and ", nextCity['name']);
                });
            }
        } else if (this.cities.length === 1) {
            this.cities[0]['transitionTime'] = ""
        }
    },
    updateCityTransition(city, value) {
        let tTime = value['rows'][0]['elements'][0]['duration']['value'];
        if (tTime) {
            city['transitionTimeRaw'] = tTime;
            var duration = m.duration(city['transitionTimeRaw'], 'seconds');
            //console.log(duration);
            city['transitionTime'] = (duration.hours() + duration.days()*24) + ' hours ' + duration.minutes() + ' min';
        } else {
            city['transitionTimeRaw'] = -1;
            city['transitionTime'] = "-";
        }
    },
    getCityDistance: function(from, to) {
        //console.log('Getting city distance');
        //console.log(from);
        var fromId = from['place_id'];
        var toId = to['place_id'];
        //console.log("From ", fromId, " to ", toId);
        var query = { from: encodeURI(fromId), to: encodeURI(toId) };
        var url = '/api/citydistance';

        //console.log(url);
        return this.$http.post(url, query).then(
            function (response) {
                if (response.ok) {
                    return response.data;
                }
            }, function (err) {
                //console.log(err);
            }
        );

    },
    saveRoadtrip: function() {
        //console.log("Saving roadtrip...");
        var data = {cities: [], date: []};
        this.cities.forEach(function (c, i) {
            data['cities'].push(c);
        });

        data['startdate'] = this.startdate
        data['enddate'] = this.enddate

        //console.log(data);

        this.$http.post('/api/save', data).then(
            function (response) {
                //console.log(response.data)
                this.planID = response.data
                this.showAlert = true
            },
            function (err) {
                //console.log(err);
            }
        )
    }
  },
  events: {
    'update-transitions': function() {
        this.updateCityTransitions()
    }
  },
  components: {
    //Map,
    Timeline,
    City
  }
};
</script>

<style>
map {
  width:600px;
  height: 300px;
  display: block;
}
</style>
