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
      input(form="POST", placeholder="San Francisco", v-model="city")
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
      var city = { name: this.city, activities: [], count: 1, transitionTime: '-', nextCity: '' }
      this.cities.push(city)
      if (this.cities.length > 1) {
          var prevCity = this.cities[this.cities.length - 2];
          prevCity['nextCity'] = city;
          var res = this.getCityDistance(prevCity['name'], city['name']);
          res.then(function(value) {
             prevCity['transitionTime'] =value['rows'][0]['elements'][0]['duration']['text'];
             console.log(prevCity);
          }, function(value) {
              console.log("Failed to get city distance");
          });
      }
      this.city = null
      addCity(this.$store, city)
    },
    getCityDistance: function(from, to) {
        console.log(encodeURI(from));
        return this.$http.get('http://localhost:8080/api/citydistance?'
          + 'from=' + encodeURI(from)
          + '&to=' + encodeURI(to),
          function(data, status, request) {
              return data;
          }
        );
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
