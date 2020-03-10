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
    keys[ev.code] = true
    checkReleased()
})

document.addEventListener("keyup", ev => {
    keys[ev.code] = false
    checkReleased()
})

export const keyPressed = key => {
    const ret = !!keys[key]
    checkReleased()
    return ret
}
export const keyReleased = key => {
    const ret = !!released[key]
    checkReleased()
    return ret
}
