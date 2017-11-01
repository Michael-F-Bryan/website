import { createStore, applyMiddleware } from 'redux'
import { createLogger } from 'redux-logger'

const initialState = {}

function reducer(state = initialState, action) {
    console.log(action)
    return state
}

const middleware = applyMiddleware(createLogger())
export default createStore(reducer, middleware)