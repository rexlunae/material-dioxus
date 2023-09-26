import fs from 'fs'

let core_js = fs.readFileSync('./build/core.js').toString()

// customization of textfield icon color
core_js = core_js.replace(
    /(\.mdc-text-field:not\(\.mdc-text-field--disabled\) \.mdc-text-field__icon--(?:leading|trailing){color:)(rgba\(0, 0, 0, 0\.54\))(})/g,
    '$1var(--mdc-text-field-icon-color, $2)$3',
).replace(
    /(\.mdc-text-field--disabled \.mdc-text-field__icon--(?:leading|trailing){color:)(rgba\(0, 0, 0, 0\.3\))(})/g,
    '$1var(--mdc-text-field-disabled-icon-color, $2)$3',
)

// allow using native WebKit calendar picker
core_js = core_js.replace(
    /(\.mdc-text-field__input)(::-webkit-calendar-picker-indicator)({display:none})/g,
    '$1$2$3.mdc-text-field--webkit-date-picker $1{position:relative}.mdc-text-field--webkit-date-picker $1$2{display:inline-block;position:absolute;inset:0;width:auto;height:auto;color:transparent;background:transparent}',
).replace(
    /(r\(\[c\({type:Boolean)(,reflect:!0)(}\)],gr\.prototype,")(disabled)(",void 0\))/g,
    '$1$2$3$4$5,$1$3webkitDatePicker$5',
).replace(
    /(this\.iconTrailing="",this\.disabled=!1)/g,
    '$1,this.webkitDatePicker=!1',
).replace(
    /("mdc-text-field--disabled":this\.disabled)/g,
    '$1,"mdc-text-field--webkit-date-picker":this.webkitDatePicker',
)

// fix textarea internal char counter color
core_js = core_js.replace(
    /(\.mdc-text-field-character-counter{color:)(rgba\(0, 0, 0, 0\.6\))(})/g,
    '$1var(--mdc-text-field-label-ink-color, $2)$3',
)

fs.writeFileSync('./build/core.js', core_js)

let list_js = fs.readFileSync('./build/mwc-list.js').toString()

// customization of list divider color
list_js = list_js.replace(
    /(\.mdc-deprecated-list ::slotted\(\[divider\]\){[^\}]*border-bottom-color:)(rgba\(0, 0, 0, 0\.12\))(})/g,
    '$1var(--mdc-deprecated-list-divider-color, $2)$3',
)

fs.writeFileSync('./build/mwc-list.js', list_js)
