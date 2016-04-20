import Vue          from 'vue'
import VueRouter    from 'vue-router'
import VueResource  from 'vue-resource'

Vue.use(VueRouter)
Vue.use(VueResource)

import App from './App.vue'


var router = new VueRouter()

router.map({
    '/': {
        component: require('./components/Start.vue')
    },
    '/plan': {
        component: require('./components/Planner.vue')
    },
    '/city': {
        component: require('./components/City.vue')
    },
    '/roadtrip': {
        component: {
            tmplate: '405 - lel' //TODO FIX ME PRKL
        }
    },
    '*': {
        component: {
            template: '404 - Not found'
        }
    }
})

router.start(App, '#app')
