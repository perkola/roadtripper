<template lang="jade">
//
  map(
      :center.sync="center",
      :zoom.sync="zoom",
      :map-type-id.sync="mapType"
  )

div.planner
  div.roadtrip-cities(v-for="city in cities")
    div.roadtrip-city
      div.roadtrip-city__content(v-on:click="showCity")
        h2 {{ city.name }}
        h3 Activities
        div.roadtrip-city__activities(v-for="act in city.activities")
          {{ act.name }} #[i.roadtrip-city__activities__location at {{ act.location }}]
        input(form="POST").roadtrip-city__add-activity
        i.material-icons favorite
      div.roadtrip-city__transition
        //i.material-icons.roadtrip-city__transition__icon arrow forward
        hr.roadtrip-city__transition__travel-line
        span.roadtrip-city__transition__time 13h37m

  div.roadtrip-cities
    div.roadtrip-city
      div.roadtrip-city__content(v-on:click="addCity")
        h2 Add city
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
      console.log("Adding a new city")
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
