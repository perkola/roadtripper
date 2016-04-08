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
    '*': {
        component: {
            template: '404 - Not found'
        }
    }
})

router.start(App, '#app')
