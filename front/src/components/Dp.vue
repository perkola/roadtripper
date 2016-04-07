<template lang="jade">
div.datepicker
    div.datepicker__title(@click="show = !show") {{ display }}
    div.datepicker__selected(@click="show = !show")
        {{ selected.format('MMMM, D - YYYY') }}
        #[i.material-icons date_range]

    div.datepicker__mask(v-show="show", transition="modal")
        div.datepicker__wrapper
            div.datepicker__container
                a.datepicker__close(@click="show = false") #[i.material-icons close]
                div.datepicker__heading
                    h1 {{ display }}
                    span(@click.prevent.stop="prev()") #[i.material-icons keyboard_arrow_left]
                    span {{ currentMonth }}
                    span(@click.prevent.stop="next()") #[i.material-icons keyboard_arrow_right]

                div.datepicker__calendar
                    div.datepicker__calendar__weekdays
                        div.datepicker__calendar__weekday(v-for="day in weekDays") {{ day }}

                    div.datepicker__calendar__days
                        div.datepicker__calendar__day(
                            v-for="day in calendarDays",
                            :class="{ 'datepicker__calendar__day--shade': !isCurrentMonth(day), 'datepicker__calendar__day--today': isToday(day), 'datepicker__calendar__day--selected': isSelected(day) }",
                            @click="selection(day)"
                        )
                            span {{ day.format('D') }}
</template>

<script>
import m           from 'moment'
import DateRange   from 'moment-range'
import { setDate } from '../vuex/actions'

export default {

    props: {
        start: {
            type: String,
            default: m().format('YYYY-MM-DD')
        },
        startFormat: {
            type: String,
            default: 'YYYY-MM-DD'
        },
        selected: {
            default: m()
        },
        name: {
            type: String,
        },
        display: {
            type: String
        }
    },

    data () {
        return {
            show: false,
            //selected: m()
        }
    },

    computed: {
        currentMonth () {
            return this.date().format('MMMM, YYYY')
        },
        weekDays () {
            return m.weekdaysShort()
        },
        calendarDays () {
            var start = this.date().startOf('month').startOf('week')
            var end  = this.date().endOf('month').endOf('week')
            var range = m.range(start, end)
            var days  = []

            range.by('days', function(moment) {
                days.push(moment)
            })

            return days
        },
    },

    methods: {
        date () {
            return m(this.start, this.startFormat)
        },
        next (type = 'M', select = false) {
            var m = this.date().add(1, type)
            this.$set('start', m.format(this.startFormat))
            if (select)
                this.$set('selected', m)
        },
        prev (type = 'M',select = false) {
            var m = this.date().subtract(1, type)
            this.$set('start', m.format(this.startFormat))
            if (select)
                this.$set('selected', m)
        },
        selection (moment) {
            this.$set('selected', moment)
            this.$set('start', moment)
            this.$set('show', false)
            document.activeElement.blur()
            setDate(this.$store, this.name, moment)
        },
        isToday (moment) {
            return moment.isSame( m(), 'day' )
        },
        isCurrentMonth (moment) {
            return moment.isSame( this.start, 'month' )
        },
        isSelected (moment) {
            return moment.isSame( this.selected, 'day' )
        },
        blur () {
            console.log(arguments, blur);
        }
    }
}
</script>
