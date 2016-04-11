<template lang="jade">
div.intro
    h1.intro__title Roadtripper
    div.col-75
        dp(name="startdate", display="Start date")
    dov.col-25
        a.intro__go(v-link="{ path: 'plan' }").go-btn #[span GO] #[i.material-icons keyboard_arrow_right]

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
