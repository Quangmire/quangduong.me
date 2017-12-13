import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom'
import {
    Card,
    CardTitle,
    CardHeader,
    CardBody
} from 'reactstrap';

class About extends React.Component {

    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div className='about'>
                <Card>
                    <CardBody>
                        <img className='about-img' src='/static/media/profile.png'/>
                        <h3>Quang Duong</h3>
                        <p><strong>Bio:</strong> Quang is a third year CS/Math student with an addiction to reading anything from fantasy to romance.</p>
                    </CardBody>
                </Card>
            </div>
        );
    }
}

export default About;
