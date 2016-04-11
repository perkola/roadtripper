import Vue  from 'vue'
import Vuex from 'vuex'
import m    from 'moment'

Vue.use(Vuex)

const state = {
    dates: {
        'startdate': m()
    },
    startdate: m(),
    duration: 10,
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
    }
}

export default new Vuex.Store({
    state,
    mutations
})
