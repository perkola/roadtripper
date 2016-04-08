import Vue  from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const state = {
    startdate: null,
    enddate: null
}

const mutations = {
    SET_START_DATE(state, moment) {
        state.startdate = moment
    },
    SET_END_DATE(state, moment) {
        state.enddate = moment
    }
}

export default new Vuex.Store({
    state,
    mutations
})
