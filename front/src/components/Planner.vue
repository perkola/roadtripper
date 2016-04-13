<template lang="jade">
//
  map(
      :center.sync="center",
      :zoom.sync="zoom",
      :map-type-id.sync="mapType"
  )
div.planner
  div.planner__add-city
    div.planner__add-city__input
      input(form="POST", placeholder="San Francisco", v-model="city", @keyup.enter="addCity")
      button(v-on:click="addCity") Add city

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
  methods: {
    showCity: function(e) {
      console.log(e)
    },
    addCity: function() {
      console.log(this.cities)
      var city = { name: this.city, activities: [], count: 1 }
      this.cities.push(city)
      this.city = null
      addCity(this.$store, city)
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
