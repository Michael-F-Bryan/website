import React from "react";
import ReactDOM from "react-dom";
import 'bootstrap/dist/css/bootstrap.min.css';
import { Provider } from "react-redux";
import Main from "./Components/Main";
import store from "./store";

import "./index.css";
import 'bootstrap/dist/css/bootstrap.min.css';

const rootElement = document.getElementById("root");
ReactDOM.render(
  <Provider store={store}>
    <Main />
  </Provider>,
  rootElement
);
