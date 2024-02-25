import React, { useState } from 'react';

interface SubmissionData {
  name: string;
  strategy: number[];
}

interface Winner {
  score: number;
  winner: string;
}

const SubmissionComponent: React.FC = () => {
  const [name, setName] = useState<string>('');
  const [strategy, setStrategy] = useState<number[]>([]);
  const [winner, setWinner] = useState<string>('');
  const [score, setScore] = useState<number>(0);


  const handleSubmit = async () => {
    const submissionData: SubmissionData = { name, strategy };

    try {
      const response = await fetch('http://localhost:8080/submission', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(submissionData),
      });

      if (response.ok) {
        console.log('Submission successful');
        // Handle success, e.g., show a success message to the user
      } else {
        console.error('Submission failed');
        // Handle failure, e.g., show an error message to the user
      }
    } catch (error) {
      console.error('Error submitting data:', error);
      // Handle network errors or other issues
    }
  };

  const handleViewWinner = async () => {
    {
      const response = await fetch('http://localhost:8080/winner', {
        method: 'GET',
      });
      if (response.ok) {
        console.log("game has ended");
        const output = await response.json();
        setWinner(output.winner);
        setScore(output.score);
      }
    }
  }

  return (
    <div>
      <label>
        Name:
        <input type="text" value={name} onChange={(e) => setName(e.target.value)} />
      </label>
      <br />
      <label>
        Strategy (comma-separated integers):
        <input
          type="text"
          value={strategy.join(',')}
          onChange={(e) => setStrategy(e.target.value.split(',').map(Number))}
        />
      </label>
      <br />
      <button onClick={handleSubmit}>Submit</button>
      <div>
        <p></p>
        <button onClick={handleViewWinner}>View Winner</button>
          <h3>Winner is: {winner} with a score:  {score}</h3>
      </div>
    </div>
  );
};

export default SubmissionComponent;
