export const setDate = ({ dispatch }, name, moment) => {
    if (name === "startdate") {
        dispatch("SET_START_DATE", moment)
    } else {
        dispatch("SET_END_DATE", moment)
    }
}

export const addCity = ({ dispatch }, city) => {
    dispatch("ADD_CITY", city)
}
