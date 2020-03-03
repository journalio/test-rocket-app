import Vue from 'vue'
import TestRocketApp from './TestRocketApp'

import './style.css'

const ROOT_EL = (() => {
    const el = document.createElement('div')
    el.id = '#app'
    document.body.appendChild(el)
    return el
})()

new Vue({
    el: ROOT_EL,
    render: h => h(TestRocketApp),
})