import Vue       from 'vue'
import VueRouter from 'vue-router'

Vue.use(VueRouter)

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
    '*': {
        component: {
            template: '404 - Not found'
        }
    }
})

router.start(App, '#app')
