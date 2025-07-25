# Apronymer Project - AI Agent Instructions

## 🎯 Project Overview

Apronymer is a full-stack web application that generates acronyms (apronyms) from related terms. The application consists of a Rust backend API and a React TypeScript frontend, deployed to Azure Container Apps with automated CI/CD via GitHub Actions.

> **Note**: This documentation was generated on July 25, 2025 to guide AI coding agents working on this project.

### Core Functionality
- **Input**: User provides related terms (3-10), fragment length (1-3), min/max length constraints
- **Processing**: Backend generates apronyms using dictionary lookup and validation
- **Output**: List of valid apronyms with explanations and copy/download functionality

## 🏗️ Architecture

### Backend (Rust/Axum)
- **Framework**: Axum web framework with Tokio async runtime
- **Structure**: Modular design with separate concerns (routes, validation, generation, dictionary)
- **Authentication**: API key middleware via `x-api-key` header
- **Logging**: Structured logging with tracing/tracing-subscriber
- **Dictionary**: Word list loaded from `wordlist/words.txt`

### Frontend (React/TypeScript/Vite)
- **Framework**: React 19 with TypeScript and Vite build system
- **Styling**: Tailwind CSS for responsive design
- **State Management**: React hooks (useState, useEffect)
- **API Communication**: Fetch API with proper error handling
- **Mobile Support**: Touch-friendly range sliders and responsive layout

### Infrastructure
- **Deployment**: Azure Container Apps with multi-stage Docker builds
- **CI/CD**: GitHub Actions with manual dispatch and configurable targets
- **Containerization**: Separate Dockerfiles for frontend/backend optimization

## 📁 Project Structure

```
apronymer/
├── .github/workflows/deploy-to-aca.yml    # CI/CD pipeline
├── backend/                               # Rust API server
│   ├── src/
│   │   ├── main.rs                       # Server entry point, middleware
│   │   ├── routes.rs                     # API endpoints
│   │   ├── validator.rs                  # Request validation logic
│   │   ├── generator.rs                  # Apronym generation algorithm
│   │   └── dictionary.rs                 # Dictionary loading/lookup
│   ├── wordlist/words.txt                # Dictionary file
│   ├── Dockerfile                        # Multi-stage build
│   ├── deploy-to-aca.sh                  # Deployment automation
│   └── Cargo.toml                        # Rust dependencies
├── frontend/                             # React application
│   ├── src/
│   │   ├── App.tsx                       # Root component
│   │   ├── ApronymForm.tsx               # Main form logic
│   │   ├── FormField.tsx                 # Desktop input components
│   │   ├── SliderField.tsx               # Mobile-friendly range inputs
│   │   ├── ResultsList.tsx               # Results display
│   │   ├── useApronymValidation.ts       # Form validation hook
│   │   └── config.ts                     # Environment configuration
│   ├── Dockerfile                        # Nginx + React build
│   └── package.json                      # Node dependencies
└── Cargo.toml                            # Workspace configuration
```

## 🔧 Development Workflow

### Local Development
1. **Backend**: Use VS Code task "Run Backend" or `cargo run` in `/backend`
2. **Frontend**: Use VS Code task "Run Frontend" or `npm run dev` in `/frontend`
3. **Environment**: Backend requires `API_KEY` and `RUST_LOG=debug` environment variables

### Build & Test
- **Backend Build**: `cargo build` (produces optimized binary)
- **Frontend Build**: `npm run build` (outputs to `dist/`)
- **Docker**: Multi-stage builds for production optimization
- **Testing**: `cargo test` for backend unit tests

### Deployment
- **Script**: `./backend/deploy-to-aca.sh` with flags `--frontend`, `--backend`, `--both`, `--clean`
- **CI/CD**: GitHub Actions with manual dispatch and environment selection
- **Environments**: Uses GitHub secrets for API keys and Azure credentials

## 🛡️ Security & Validation

### API Security
- **Authentication**: All `/api/*` routes require `x-api-key` header
- **CORS**: Configured for cross-origin requests with specific headers
- **Rate Limiting**: Not yet implemented (planned feature)

### Input Validation
- **Terms**: 3-10 unique terms required
- **Fragment Length**: 1-3 characters per fragment
- **Length Constraints**: Min/max length between 1-10, max ≤ number of terms
- **Complexity Limits**: Prevents excessive combinations (max 10,000)

### Error Handling
- **Backend**: Structured error responses with appropriate HTTP status codes
- **Frontend**: User-friendly error messages and validation feedback
- **Logging**: Comprehensive tracing for debugging and monitoring

## 🎨 UI/UX Patterns

### Responsive Design
- **Mobile-First**: Touch-friendly sliders for numeric inputs
- **Desktop**: Traditional form fields with validation
- **Responsive Layout**: Tailwind CSS with responsive breakpoints

### Form Behavior
- **Dynamic Ranges**: Slider limits adjust based on input constraints
- **Validation**: Real-time feedback with error messages
- **State Management**: Prevents duplicate submissions, maintains form state
- **Results**: Copy-to-clipboard and download functionality

### Component Architecture
- **ApronymForm**: Main form container with state management
- **FormField**: Desktop text/number inputs with validation
- **SliderField**: Mobile-optimized range inputs
- **ResultsList**: Results display with interactive features

## 🔄 API Contract

### Endpoints
- `GET /health` - Health check (no auth required)
- `POST /api/generate` - Generate apronyms (requires API key)

### Request Format
```json
{
  "terms": ["term1", "term2", "term3"],
  "frag_len": 2,
  "min_len": 3,
  "max_len": 4
}
```

### Response Format
```json
[
  {
    "apronym": "EXAMPLE",
    "explanation": "EX-AM-PLE explanation",
    "fragments": ["EX", "AM", "PLE"]
  }
]
```

## 🚀 Deployment Configuration

### Environment Variables
- **Backend**: `API_KEY`, `HOST`, `PORT`, `RUST_LOG`
- **Frontend**: `VITE_API_KEY`, `VITE_API_BASE_URL`
- **Build-time**: API key injected during Docker build for production

### Azure Container Apps
- **Backend**: Exposes port 3000, health checks enabled
- **Frontend**: Nginx serving static files on port 80
- **Networking**: Internal communication and external access configured

### Docker Optimization
- **Multi-stage builds**: Separate build and runtime stages
- **Size optimization**: Minimal runtime images with security updates
- **Platform targeting**: Linux/amd64 for consistency

## 🧪 Testing Strategy

### Backend Testing
- **Unit Tests**: Core validation and generation logic
- **Integration**: API endpoint testing with mock data
- **Environment**: Test environment variables and configuration

### Frontend Testing
- **Component Testing**: Form validation and user interactions
- **API Integration**: Mock API responses and error scenarios
- **Responsive**: Cross-device and cross-browser testing

## 🔍 Debugging & Monitoring

### Logging Strategy
- **Structured Logging**: Tracing with file/line information
- **Log Levels**: Debug for development, info/warn/error for production
- **Request Tracing**: HTTP request/response logging with correlation IDs

### Error Diagnosis
- **Backend Errors**: Check container logs via Azure CLI or portal
- **Frontend Issues**: Browser dev tools and network tab
- **Deployment**: GitHub Actions logs and deployment script output

## 📋 Development Conventions

### Code Style
- **Rust**: Standard cargo fmt and clippy conventions
- **TypeScript**: ESLint configuration with React-specific rules
- **Naming**: Descriptive function/variable names, consistent patterns

### Git Workflow
- **Branches**: Feature branches with descriptive names
- **Commits**: Clear, concise commit messages
- **PRs**: Comprehensive descriptions with context

### File Organization
- **Backend**: Modular structure with single responsibility
- **Frontend**: Component-based architecture with hooks
- **Configuration**: Environment-specific configs clearly separated

## 🎯 AI Agent Guidelines

### When Working on This Project

1. **Environment Setup**: Always check that API_KEY environment variables are properly configured
2. **Mobile Considerations**: Use SliderField components for numeric inputs on mobile devices
3. **Validation**: Implement both frontend and backend validation for security
4. **Error Handling**: Provide clear, user-friendly error messages
5. **Logging**: Add tracing statements for debugging complex operations
6. **Testing**: Verify changes work across both desktop and mobile interfaces

### Common Tasks

- **Adding Features**: Consider both frontend UX and backend validation/security
- **Performance**: Be mindful of computational complexity limits (10,000 combinations max)
- **Deployment**: Use provided VS Code tasks or deployment scripts
- **Debugging**: Check both application logs and browser developer tools

### Integration Points
- **API Communication**: Always include `x-api-key` header in requests
- **Environment Config**: Use `config.ts` for environment-specific URLs
- **Responsive Design**: Test changes on both mobile and desktop viewports
- **Docker Builds**: Verify binary functionality and size after Docker changes

This project prioritizes user experience, security, and maintainability. Focus on clean, well-documented code that follows established patterns.
