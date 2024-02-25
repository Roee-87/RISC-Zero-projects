import React, { useState } from 'react';

interface AdminInput {
  password: string;
  puzzle_size: number;
}

const AdminPage: React.FC = () => {
  const [adminInput, setAdminInput] = useState<AdminInput>({ password: '', puzzle_size: 0 });
  const [isGameStarted, setIsGameStarted] = useState(false);

  const handlePasswordChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setAdminInput({ ...adminInput, password: event.target.value });
  };

  const handlePuzzleSizeChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const size = parseInt(event.target.value, 10);
    setAdminInput({ ...adminInput, puzzle_size: isNaN(size) ? 0 : size });
  };

  const handleToggleGame = async () => {
    if (isGameStarted) {
      // If the game is already started, perform logic for ending the game
       {
        const response = await fetch('http://127.0.0.1:8080/end', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ password: adminInput.password }),
        });

        if (response.ok) {
          console.log('Game Ended by Admin');
          setIsGameStarted(false);
        } else {
          console.error('Failed to end the game.');
        }
      
      }
    } else 
        {
          const response = await fetch('http://127.0.0.1:8080/start', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify(adminInput),
          });
          setIsGameStarted(!isGameStarted);          
    }
  };

  return (
    <div>
      <h1>Admin Page</h1>
      <div>
        <label>
          Enter Password:
          <input type="password" value={adminInput.password} onChange={handlePasswordChange} />
        </label>
      </div>
      <div>
        <label>
          Enter Puzzle Size:
          <input type="number" value={adminInput.puzzle_size} onChange={handlePuzzleSizeChange} />
        </label>
      </div>
      <button onClick={handleToggleGame}>
        {isGameStarted ? 'End Game' : 'Start Game'}
      </button>
    </div>
  );
};

export default AdminPage;

// Add an empty export statement to make it a valid TypeScript module
export {}

