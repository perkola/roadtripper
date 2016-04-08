<template lang="jade">
div.timeline
    div.timeline__date(
        v-for="(index, day) in timelineDays",
        :class="this.colors[index]"
    )
        span.day {{ day.format('D') }}
        span.month {{ day.format('MMMM') }}
</template>

<script>
import m           from 'moment'
import DateRange   from 'moment-range'

export default {
    vuex: {
        getters: {
            startdate: state => state.startdate,
            enddate: state => state.enddate,
            cities: state => state.cities
        }
    },
    computed: {
        timelineDays() {
            var start = m(this.startdate, "YYYY-MM-DD")
            var end = m(this.enddate, "YYYY-MM-DD")
            var range = m.range(start, end)
            console.log(range)
            var days  = []

            range.by('days', function(moment) {
                days.push(moment)
            })

            return days
        },
        colors() {
            var colors = []
            for (var i = 0; i < this.cities.length; i++) {
                for (var j = 0; j < this.cities[i].count; j++) {
                    colors.push('color-' + i)
                }
            }
            return colors
        }
    },
    data() {
        return {
        }
    }
}
</script>
