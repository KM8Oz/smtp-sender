import test from 'ava'

import { send } from '../index.js'

test('send from native', (t) => {
    t.is(send)
})