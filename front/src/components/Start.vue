<template lang="jade">
div.intro
    h1.intro__title Roadtripper #[span.trademark TM]
    p.intro__slogan Whether you are planning to travel coast to coast in USA or a roadtrip through Europe - Roadtripper#[span.trademark TM] is the tool that will make the planning a breeze!
    div
        dp.col-50(name="startdate", display="Start date")
        dp.col-50(name="enddate", display="End date")
    dov
        a.intro__go(v-link="{ path: 'plan' }").go-btn #[span Start planning] #[i.material-icons keyboard_arrow_right]

div.footer
    p Copyright #[i.material-icons copyright] 2016
</template>

<script>
import dp from './Dp.vue'

export default {
    components: {
        dp
    }
}
</script>
