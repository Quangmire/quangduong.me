import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect
} from 'react-router-dom'
import {
    Card,
    CardBody,
    CardTitle,
    CardHeader,
    CardText
} from 'reactstrap';

class WIP extends React.Component {

    render() {
        return (
            <div className='not-found'>
                <Card>
                    <CardHeader>
                        <center>
                            <h3>This page is under construction</h3>
                        </center>
                    </CardHeader>
                    <CardBody>
                        <center>
                            <p>
                                This page does not exist yet.
                            </p>
                        </center>
                    </CardBody>
                </Card>
            </div>
        );
    }

};

export default WIP;
