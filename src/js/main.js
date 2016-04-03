var Vue       = require('vue')
var VueRouter = require('vue-router')

Vue.use(VueRouter)

var App = require('./components/App.vue')
var Start = require('./components/Start.vue')

/*
new Vue({
  el: 'body',
  components: {
    app: App
  }
})
*/

var router = new VueRouter()

router.map({
    '/': {
        component: Start
    },
    '/plan': {
        component: require('./components/Planner.vue')
    },
    '*': {
        component: {
            template: '404 - Not found'
        }
    }
})

router.start(App, '#app')
