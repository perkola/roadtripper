import Vue  from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const state = {
    startdate: null,
    enddate: null,
    cities: []
}

const mutations = {
    SET_START_DATE(state, moment) {
        state.startdate = moment
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
