import { Component } from "react";
import { withRouter } from "react-router-dom";

/**
 * A component which will let you know every time the route is changed. It will
 * never be rendered.
 */
class RouteChanged extends Component {
  componentDidMount() {
    const { history, onRouteChanged } = this.props;
    const unsub = history.listen(onRouteChanged);

    this.unsubscribe = unsub;
  }

  componentWillUnmount() {
    this.unsubscribe();
  }

  render() {
    return null;
  }
}

export default withRouter(RouteChanged);
