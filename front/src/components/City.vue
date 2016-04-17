<template lang="jade">
div.roadtrip-city
  div.roadtrip-city__content(:class="yolo")
    div.roadtrip-city__content__title
      span {{ city.name }}
      div
        button(@click="decrement()") #[i.material-icons remove]
        span {{ city.count }} days
        button(@click="increment()") #[i.material-icons add]
    div.roadtrip-city__content__content
      span.activities Activities
      div.activity(v-for="act in city.activities", transition="modal")
        {{ act.name }}
      div.roadtrip-city__content__content__add-activity
          i.material-icons.roadtrip-city__content__content__add-activity__icon add
          input(placeholder="Visit a museum", v-on:keyup.enter="addActivity", v-model="addAct").roadtrip-city__content__content__add-activity__input
    //
      input(form="POST").roadtrip-city__add-activity
      i.material-icons favorite
  div.roadtrip-city__transition
    //i.material-icons.roadtrip-city__transition__icon arrow forward
    span.roadtrip-city__transition__time
        span #[i.material-icons directions_car]
        {{ city.transitionTime }}
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
            if (this.city.count > 1) {
                this.city.count--
            } else {
                this.remove()
            }
        },
        addActivity() {
            if (this.city['activities'].length > 0) {
                this.city['activities'].push({name: this.addAct})
            } else {
                this.city['activities'] = [{name: this.addAct }]
            }
            this.addAct = ''
        }
    }
}
</script>
