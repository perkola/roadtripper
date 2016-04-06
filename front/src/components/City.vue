<template lang="jade">
div.city
  div.city__content
    div.city__content__name
      h1 {{ name }}
    div.city__content__desc
      p {{ desc }}
    div.city__activities
      div.city__activities__activity(v-for="activity in activities")
        h3 {{ activity.act }}
      div.city__activities__activity
        i(class='material-icons') add circle outline
        input(class='add-activity-input')

</template>

<script>
export default {

  data() {
      return {name: "Dallas", desc: "Where your dreams come true",
        activities: [{act: "Cykla", loc: "Dallas St. 3"},
          {act: "Hoppa hopprep", loc: "Dallas Rd. 19"},
          {act: "Springa", loc: "Dalls Avenue 192"}]}
  }

}
</script>
