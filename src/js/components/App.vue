<template lang="jade">
router-view
//timeline
</template>

<script>
import Timeline from './Timeline.vue'
import store    from '../vuex/store'

export default {
    store,
    components: {
        Timeline,
    }
}
</script>
