<template lang="jade">
div.timeline
    div.timeline__date(v-for="date in dates")
        span.day {{ date.day }}
        span.month {{ date.month }}
</template>

<script>
export default {
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
