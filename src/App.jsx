import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect,
    Switch
} from 'react-router-dom'
import $ from 'jquery';
import {
    TitleBar,
    SideBar,
    NavBar
} from './components/NavBar.jsx';
import About         from './components/About.jsx';
import Page          from './components/Page.jsx';
import NotFound      from './components/NotFound.jsx';
import WIP           from './components/WIP.jsx';

class App extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            is_open: true
        };
    }

    render() {
        return (
            <Router>
                <div id='wrapper'>
                    <SideBar/>
                    <div id='content'>
                        <NavBar />
                        <TitleBar toggle={this.toggle} />
                        <Switch>
                            <Route exact path='/about' component={About}/>
                            <Route path='/projects/:name' component={(props) => (<Page {...props} />)}/>
                            {/* These pages are not yet implemented */}
                            <Route exact path='/' component={WIP}/>
                            <Route exact path='/contact' component={WIP} />
                            <Route exact path='/projects' component={WIP} />
                            <Route exact path='/chronocides' component={WIP} />
                            <Route path='/chronocides/:name' component={WIP}/>
                            <Route exact path='/archives' component={WIP} />
                            <Route path='/archives/:name' component={WIP}/>
                            {/* 404 Page */}
                            <Route exact path='/404' component={NotFound} />
                            <Redirect to="/404" />
                        </Switch>
                    </div>
                </div>
            </Router>
        );
    }
}

/*
 *  URLs left to implement:
 <Route path='/contact' component={Contact} />
 <Route path='/projects' component={Projects} />
 <Route path='/chronocides' component={Chronocides} />
 <Route path='/chronocides/:name' component={(props) => (<Page {...props} />)}/>
 <Route path='/archives' component={Archives} />
 <Route path='/archives/:name' component={(props) => (<Page {...props} />)}/>
 */

export default App;
