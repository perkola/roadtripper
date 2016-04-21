<template lang="jade">
//
  map(
      :center.sync="center",
      :zoom.sync="zoom",
      :map-type-id.sync="mapType"
  )
div.navbar
    h1.navbar__logo #[a(v-link="{ path: '/' }") Roadtripper]
    div.navbar__city-search(:class="{ 'test': yo }")
        span(v-show="yo", transition="expand") Type the name of a city...
        input(type="text", placeholder="San Francisco", v-model="city", @keyup.enter="addNewCity")
        button(@click="addNewCity") #[i.material-icons add] Add city
    a.navbar__button(v-if="cities.length", transition="modal", @click="saveRoadtrip()") Save roadtrip #[i.material-icons exit_to_app]

div.planner

  div.planner-cities
      city(v-for="(index, city) in cities", :city="city", :class="index", transition="modal")

  Timeline

</template>

<script>

//import {load, Map } from 'vue-google-maps'
import Timeline from './Timeline.vue'
import City from './City.vue'
import { addCity } from '../vuex/actions'

//load('AIzaSyDB69x_sL1X3tawNIFdht4prhEW9bymssc', '3.23', ['places']);

export default {
  name: 'Planner',
  data: function data() {
    return {
      cities: [],
      center: { lat: 10, lng: 11 },
      zoom: 5,
      mapType: 'terrain',
      city: null
    }
  },
  vuex: {
      getters: {
          startdate: state => state.dates.startdate,
          cities: state => state.cities,
          duration: state => state.duration
      },
      actions: {
          addCity
      }
  },
  ready() {
    var id = this.$route.params.id
    if (id) {
        console.log(id)
    }
  },
  computed: {
      yo() {
          return !this.city && this.cities.length < 1
      }
  },
  methods: {
    showCity: function(e) {
      console.log(e)
    },
    autocomplete: function(e) {
        var autocomplete = this.$http.get('http://localhost:8080/api/autocomplete?input=' + this.city);
    },
    addNewCity: function() {
      console.log(this.cities)
      var city = { name: this.city, activities: [], count: 1, transitionTime: '-', nextCity: '' }
      this.cities.push(city)
      if (this.cities.length > 1) {
          var prevCity = this.cities[this.cities.length - 2];
          prevCity['nextCity'] = city;
          var res = this.getCityDistance(prevCity['name'], city['name']);
          res.then(function(value) {
             prevCity['transitionTime'] = value['rows'][0]['elements'][0]['duration']['text'];
          }, function(value) {
              console.log("Failed to get city distance");
          });
      }
      this.city = null
      addCity(this.$store, city)
    },
    /* this will update all transition times in the roadtrip */
    updateCityTransitions() {
        if (this.cities.length > 1) {
            for (var i = 0; i < this.cities.length - 1; i++) {
                var thisCity = this.cities[i];
                var nextCity = this.cities[i+1];

                thisCity['nextCity'] = nextCity;
                var res = this.getCityDistance(thisCity['name'], nextCity['name']);
                res.then(function(value) {
                    thisCity['transitionTime']
                        = value['rows'][0]['elements'][0]['duration']['text'];
                }, function (value) {
                    console.log("Failed to get city distance between ",
                        thisCity['name'], " and ", nextCity['name']);
                });
            }
        }
    },
    getCityDistance: function(from, to) {
        return this.$http.get('http://localhost:8080/api/citydistance?'
          + 'from=' + encodeURI(from)
          + '&to=' + encodeURI(to),
          function(data, status, request) {
              return data;
          }
        );
    },
    saveRoadtrip: function() {
        console.log("Saving roadtrip");
        var data = {cities: [], date: []};
        this.cities.forEach(function (c, i) {
            data['cities'].push(c);
        });

        data['date'].push({'startDate' : this.startdate});
        data['date'].push({'duration' : this.duration});

        console.log(data);

        this.$http.post('http://localhost:8080/api/save', data).then(
            function (data) {
                console.log(data['data']);
            },
            function (err) {
                console.log(err);
            }
        )
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
