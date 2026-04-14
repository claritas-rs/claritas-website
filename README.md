<div align="center">

<img width="100%" src="https://capsule-render.vercel.app/api?type=waving&color=222222&height=200&section=header&text=Claritas%20Website&fontSize=50&fontColor=fff&animation=twinkling&fontAlignY=40&desc=The%20landing%20page%20for%20the%20Claritas%20EPUB%20and%20PDF%20reader&descAlignY=60&descSize=18">

<p align="center">
  <i>A blazing fast, minimalist, and responsive landing page built entirely in Rust using Leptos and Tailwind CSS.</i>
</p>

---

### Features

<div align="center">

| Feature | Description                                                                  |
| :-----: | :--------------------------------------------------------------------------- |
|   Fast  | Compiled to WebAssembly for near-instant load times and interactions         |
|  Design | Minimalist, terminal-inspired dark theme matching the Claritas app           |
|  Anim   | Smooth scroll animations, lightbox gallery, and interactive UI feedback      |
|  Stack  | 100% Rust frontend using Leptos (CSR) + Tailwind v4 for modern styling       |

</div>

---

### Technologies

<div align="center">
  <a href="https://skillicons.dev">
    <img src="https://skillicons.dev/icons?i=rust,wasm,tailwind,html,css&theme=dark" />
  </a>
</div>

<div align="center">
  <sub>Rust • Leptos • WebAssembly • Tailwind CSS • Trunk</sub>
</div>

---

### Structure

```text
claritas-website/
├── .github/workflows/          
│   └── deploy.yml              # GitHub Actions CI/CD for GitHub Pages
├── input.css                   # Tailwind base styles and custom theme variables
├── index.html                  # HTML entry point and global scripts
└── src/                        
    ├── app.rs                  # Main layout and routing
    ├── components/             # Reusable UI components
    │   ├── hero.rs             # Main hero section with interactive gallery
    │   ├── features.rs         # Feature highlights
    │   ├── download.rs         # Download CTA and links
    │   ├── stats.rs            # Application statistics
    │   └── navbar.rs           # Responsive navigation
    └── main.rs                 # App mount point
```

---

### Getting Started

#### Development

```bash
# Clone the repository
git clone https://github.com/claritas-rs/claritas-website.git
cd claritas-website

# Install Trunk (Rust WASM application bundler)
cargo install trunk

# Run the development server (auto-reloads on changes)
trunk serve --open
```

> **Note:** The development server usually runs on `http://localhost:8080`.

#### Build for Production

To create a release build of the website:
```bash
trunk build --release
```
The compiled HTML, CSS, JS, and WebAssembly files will be generated in the `dist/` directory, ready to be hosted on any static file server.

---

### Deployment

This website is automatically deployed to **GitHub Pages** using a GitHub Actions workflow. Every push to the `main` branch triggers a build using Trunk, which then publishes the `dist/` folder.

---

### Author

<div align="center">
    <a href="https://github.com/matheussricardoo" target="_blank">
        <img src="https://skillicons.dev/icons?i=github" alt="GitHub"/>
      </a>
      <a href="https://www.linkedin.com/in/matheus-ricardo-426452266/" target="_blank">
        <img src="https://skillicons.dev/icons?i=linkedin" alt="LinkedIn"/>
      </a>
</div>

<img width="100%" src="https://capsule-render.vercel.app/api?type=waving&color=222222&height=120&section=footer"/>

</div>