import React, { useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import UserSubmission from './components/UserSubmission';
import AdminPage from './components/AdminPage';




  const App: React.FC = () => {
    return (
      <Router>
        <div>
          <Routes>
            <Route path="/admin" element={<AdminPage />} />
            <Route path="/user-submission" element={<UserSubmission />} />
          </Routes>
        </div>
      </Router>
    );
  };

export default App;
