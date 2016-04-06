<template lang="jade">
div.datepicker
    div.datepicker__selected(@click="visible = !visible") {{ selected.format('MMMM, D - YYYY') }}

    div.datepicker__calendar(v-if="visible")
        div.calendar__heading
            button(@click.prevent.stop="prev()") &laquo;
            div.calendar_current-month {{ currentMonth }}
            button(@click.prevent.stop="next()") &raquo;

        div.calendar
            div.calendar__weekdays
                div.calendar__weekday(v-for="day in weekDays") {{ day }}

            div.calendar__days
                div.calendar__day(
                    v-for="day in calendarDays",
                    :class="{ 'calendar__day--shade': !isCurrentMonth(day), 'calendar__day--today': isToday(day), 'calendar__day--selected': isSelected(day) }",
                    @click="selection(day)"
                )
                    span {{ day.format('D') }}
</template>

<script>
import m           from 'moment'
import DateRange   from 'moment-range'
import { setDate } from '../vuex/actions'

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
        }
    },

    data () {
        return {
            visible: false,
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
            this.$set('visible', false)
            document.activeElement.blur()
            localStorage.setItem(this.name, moment)
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
