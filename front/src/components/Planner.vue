<template lang="jade">
//
  map(
      :center.sync="center",
      :zoom.sync="zoom",
      :map-type-id.sync="mapType"
  )
div.planner
  div.planner-cities
    div.roadtrip-cities(v-for="city in cities")
      div.roadtrip-city
        div.roadtrip-city__content(v-on:click="getCityDistance")
          div.roadtrip-city__content__title
            {{ city.name }}
          div.roadtrip-city__content__content
            Activities
            div.roadtrip-city__activities(v-for="act in city.activities")
              {{ act.name }} #[i.roadtrip-city__activities__location at {{ act.location }}]
          //
            input(form="POST").roadtrip-city__add-activity
            i.material-icons favorite
        div.roadtrip-city__transition
          //i.material-icons.roadtrip-city__transition__icon arrow forward
          span.roadtrip-city__transition__time 13h37m

    div.roadtrip-cities
      div.roadtrip-city
        div.roadtrip-city__content(v-on:click="addCity")
          div.roadtrip-city__content__title
            | Add city
          div.roadtrip-city__content__add-circle
            i.material-icons.md-48 add_circle_outline

  Timeline

</template>

<script>

import {load, Map } from 'vue-google-maps'
import Timeline from './Timeline.vue'

load('AIzaSyDB69x_sL1X3tawNIFdht4prhEW9bymssc', '3.23', ['places']);

export default {
  data: function data() {
    return {
      cities: [{ name: 'Dallas', activities: [{name: "Cykla", location: "Harbour"}, {name: "Jump", location: "The Great Wheel"}]}, { name: 'San Francisco', activities: [{name: "Springa", location: "The Beach"}]}],
      center: { lat: 10, lng: 11 },
      zoom: 5,
      mapType: 'terrain',
    };
  },
  methods: {
    showCity: function(e) {
      console.log(e)
    },
    addCity: function() {
      console.log(this.cities)
      var city = { name: "Stockholm", activities: [{name: "Roll", location: "KTH"}]}
      this.cities.push(city)
    },
    getCityDistance: function() {
      makeCorsRequest()
      function createCORSRequest(method, url) {
        var xhr = new XMLHttpRequest();
        if ("withCredentials" in xhr) {
          // XHR for Chrome/Firefox/Opera/Safari.
          xhr.open(method, url, true);
        } else if (typeof XDomainRequest != "undefined") {
          // XDomainRequest for IE.
          xhr = new XDomainRequest();
          xhr.open(method, url);
        } else {
          // CORS not supported.
          xhr = null;
        }
        return xhr;
      }

      function makeCorsRequest() {
        // All HTML5 Rocks properties support CORS.
        var url = 'http://localhost:8000/api/citydistance?from=dallas&to=san%20francisco';

        var xhr = createCORSRequest('GET', url);
        if (!xhr) {
          alert('CORS not supported');
          return;
        }

        // Response handlers.
        xhr.onload = function() {
          var text = xhr.responseText;
          alert('Response from CORS request to ' + url);
        };

        xhr.onerror = function() {
          alert('Woops, there was an error making the request.');
        };

        xhr.send();
      }

    }
  },
  components: {
    Map,
    Timeline
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
