<template lang="jade">
div.timeline
    div.timeline__date(v-for="day in timelineDays")
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
            enddate: state => state.enddate
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
    },

    ready() {
        console.log(this.timelineDays)
    },
    data() {
        return {
            dates: [
                {
                    'day': '02',
                    'month': 'June'
                },
                {
                    'day': '03',
                    'month': 'June'
                },
                {
                    'day': '04',
                    'month': 'June'
                },
                {
                    'day': '05',
                    'month': 'June'
                },
                {
                    'day': '06',
                    'month': 'June'
                },
                {
                    'day': '07',
                    'month': 'June'
                },
                {
                    'day': '08',
                    'month': 'June'
                },
                {
                    'day': '09',
                    'month': 'June'
                },
                {
                    'day': '10',
                    'month': 'June'
                },
                {
                    'day': '11',
                    'month': 'June'
                },
                {
                    'day': '12',
                    'month': 'June'
                },
                {
                    'day': '13',
                    'month': 'June'
                },
                {
                    'day': '14',
                    'month': 'June'
                },
                {
                    'day': '15',
                    'month': 'June'
                },
                {
                    'day': '16',
                    'month': 'June'
                },
                {
                    'day': '17',
                    'month': 'June'
                },
                {
                    'day': '18',
                    'month': 'June'
                },
                {
                    'day': '19',
                    'month': 'June'
                },
            ]
        }
    }
}
</script>
