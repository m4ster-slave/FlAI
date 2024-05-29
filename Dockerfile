# Use an official Node.js runtime as a parent image
FROM node:16

# Set the working directory
WORKDIR /flai_rs

# Copy package.json and package-lock.json to the working directory
COPY www/package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code to the working directory
COPY . .

# Set environment variable
ENV NODE_OPTIONS=--openssl-legacy-provider

# Expose the port your app runs on
EXPOSE 42069

# Define the command to run your app
CMD ["npm", "run", "start", "--", "--host", "0.0.0.0", "--port", "42069"]

