<template lang="jade">
div.bg-img
div.intro
    div.intro__content
        h1.intro__title Roadtripper
        div.intro__pick
            div
                div.date-from
                    dp(name="startdate", display="Start date")
                div.date-to
                    dp(name="enddate", display="End date")
            p.intro__go
                a(v-link="{ path: 'plan' }").go-btn Create roadtrip
p.footer Copyright &copy; 2016
</template>

<script>
import dp from './Dp.vue'

export default {
    components: {
        dp
    }
}
</script>
