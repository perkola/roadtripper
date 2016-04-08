<template lang="jade">
div.roadtrip-city
  div.roadtrip-city__content(:class="yolo", v-on:click="getCityDistance")
    div.roadtrip-city__content__title
      span {{ city.name }}
      div
        button(@click="decrement()") #[i.material-icons remove]
        span {{ city.count }} days
        button(@click="increment()") #[i.material-icons add]
    div.roadtrip-city__content__content
      | Activities
      div.roadtrip-city__activities(v-for="act in city.activities")
        {{ act.name }} #[i.roadtrip-city__activities__location at {{ act.location }}]
    //
      input(form="POST").roadtrip-city__add-activity
      i.material-icons favorite
  div.roadtrip-city__transition
    //i.material-icons.roadtrip-city__transition__icon arrow forward
    span.roadtrip-city__transition__time 13h37m
//
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
    props: {
        city: {
            type: Object,
            require: true
        },
        class: {

        }
    },
    computed: {
        yolo() {
            return 'color-' + this.class
        }
    },
    data() {
        return {
            count: 1
        }
    },
    methods: {
        increment() {
            this.city.count++
        },
        decrement() {
            if (this.city.count > 0) {
                this.city.count--
            }
        }
    }
}
</script>
