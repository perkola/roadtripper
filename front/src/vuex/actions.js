export const setDate = ({ dispatch }, name, moment) => {
    console.log(moment._i)
    if (name === "startdate") {
        dispatch("SET_START_DATE", moment)
    } else {
        dispatch("SET_END_DATE", moment)
    }
}
