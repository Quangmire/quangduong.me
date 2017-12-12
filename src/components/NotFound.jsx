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

class NotFound extends React.Component {

    render() {
        return (
            <div className='not-found'>
                <Card>
                    <CardHeader>
                        <center>
                            <h3>404 Page Not Found</h3>
                        </center>
                    </CardHeader>
                    <CardBody>
                        <center>
                            <p>
                                This page does not exist or wandered to a new home.
                            </p>
                            <p>
                                If you think this is an error, <Link to='/contact'>let me know</Link> so I can fix it! :D
                            </p>
                        </center>
                    </CardBody>
                </Card>
            </div>
        );
    }

};

export default NotFound;
