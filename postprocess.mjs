import fs from 'fs'

let core_js = fs.readFileSync('./material-dioxus/build/core.js').toString()

// customization of textfield icon color
core_js = core_js.replace(
    /(\.mdc-text-field:not\(\.mdc-text-field--disabled\) \.mdc-text-field__icon--(?:leading|trailing){color:)(rgba\(0, 0, 0, 0\.54\))(})/g,
    '$1var(--mdc-text-field-icon-color, $2)$3',
).replace(
    /(\.mdc-text-field--disabled \.mdc-text-field__icon--(?:leading|trailing){color:)(rgba\(0, 0, 0, 0\.3\))(})/g,
    '$1var(--mdc-text-field-disabled-icon-color, $2)$3',
)

fs.writeFileSync('./material-dioxus/build/core.js', core_js)
