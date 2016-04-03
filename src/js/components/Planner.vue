<template lang="jade">
map(
    :center.sync="center",
    :zoom.sync="zoom",
    :map-type-id.sync="mapType"
)
</template>

<script>

import {load, Map } from 'vue-google-maps'

load('AIzaSyDB69x_sL1X3tawNIFdht4prhEW9bymssc', '3.23', ['places']);

export default {
  data: function data() {
    return {
      center: { lat: 10, lng: 11 },
      zoom: 5,
      mapType: 'terrain',
    };
  },
  components: {
    Map
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
