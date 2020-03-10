const keys = {}

let prevKeys = {}
const released = {}
function checkReleased() {
    for (const key in released) {
        released[key] = false;
    }
    for (const key in prevKeys) {
        if (!keys[key]) released[key] = true
    }
    for (const key in keys) {
        prevKeys[key] = keys[key]
    }
}

document.addEventListener("keydown", ev => {
    keys[ev.keyCode] = true
    checkReleased()
})

document.addEventListener("keyup", ev => {
    keys[ev.keyCode] = false
    checkReleased()
})

export const key_pressed = key => {
    const ret = !!keys[key]
    checkReleased()
    return ret
}
export const key_released = key => {
    const ret = !!released[key]
    checkReleased()
    return ret
}
