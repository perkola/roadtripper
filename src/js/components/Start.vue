<template lang="jade">
div.intro
    div.intro__content
        h1.intro__title Roadtripper
        div.date-from
            h2 Start date
            dp
        div.date-to
            h2 End date
            dp
        button(v-link="{ path: 'plan' }").go-btn Go!
</template>

<script>
import dp from './Dp.vue'
import m  from 'moment'

export default {
    components: {
        dp
    }
}
</script>
