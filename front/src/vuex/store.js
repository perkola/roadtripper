import Vue  from 'vue'
import Vuex from 'vuex'
import m    from 'moment'

Vue.use(Vuex)

const state = {
    dates: {
        'startdate': m(),
        'enddate': m().add(16, 'days')
    },
    startdate: m(),
    duration: -1,
    enddate: m(),
    cities: []
}

const mutations = {
    SET_START_DATE(state, moment) {
        state.dates.startdate = moment
    },
    SET_END_DATE(state, moment) {
        state.dates.enddate = moment
    },
    ADD_CITY(state, city) {
        state.cities.push(city)
    },
    REMOVE_CITY(state, city) {
        state.cities = state.cities.filter(function(obj) {
            return obj.id !== city.id;
        });
    },
    INCREMENT_DURATION(state) {
        state.duration++
    },
    DECREMENT_DURATION(state) {
        state.duration--
    },
    SET_DURATION(state, duration) {
        state.duration = duration
    }
}

export default new Vuex.Store({
    state,
    mutations
})
