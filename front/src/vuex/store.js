import Vue  from 'vue'
import Vuex from 'vuex'
import m    from 'moment'

Vue.use(Vuex)

const state = {
    dates: {
        'startdate': m()
    },
    startdate: m(),
    duration: -1,
    enddate: m().day(16),
    cities: []
}

const mutations = {
    SET_START_DATE(state, moment) {
        state.dates.startdate = moment
    },
    SET_END_DATE(state, moment) {
        state.enddate = moment
    },
    ADD_CITY(state, city) {
        state.cities.push(city)
    },
    REMOVE_CITY(state, city) {
        state.cities = state.cities.filter(function(obj) {
            return obj.name !== city.name;
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
