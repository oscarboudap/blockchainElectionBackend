// src/services/votingService.ts
import axios from 'axios';

const backendUrl = process.env.REACT_APP_BACKEND_URL;

export const castVote = async (voterId: string, candidateId: string) => {
  return await axios.post(`${backendUrl}/vote`, { voterId, candidateId });
};

export const getResults = async () => {
  return await axios.get(`${backendUrl}/results`);
};
