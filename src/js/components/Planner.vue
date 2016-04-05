<template lang="jade">
//
  map(
      :center.sync="center",
      :zoom.sync="zoom",
      :map-type-id.sync="mapType"
  )

div.planner
  div.roadtrip-cities(v-for="city in cities")
    div.roadtrip-city(v-on:click="showCity")
      h2 {{ city.name }}
      div.roadtrip-city__activities(v-for="act in city.activities")
        {{ act.name }}
      input(form="POST").add-activity

Timeline

</template>

<script>

import {load, Map } from 'vue-google-maps'
import Timeline from './Timeline.vue'

load('AIzaSyDB69x_sL1X3tawNIFdht4prhEW9bymssc', '3.23', ['places']);

export default {
  data: function data() {
    return {
      cities: [{ name: 'Dallas', activities: [{name: "Cykla"}, {name: "Hoppa"}]}, { name: 'San Francisco', activities: [{name: "Springa"}]}],
      center: { lat: 10, lng: 11 },
      zoom: 5,
      mapType: 'terrain',
    };
  },
  methods: {
    showCity: function() {
      console.log("hej");
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
