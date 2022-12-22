// Imports

gsap.registerPlugin(ScrollTrigger)
gsap.registerPlugin(ScrollToPlugin)

// Panel with language logo

let bodyScrollBar = Scrollbar.init(document.body, {
    damping: 0.1,
    delegateTo: document,
    alwaysShowTracks: true,
})

const mainHeader = document.querySelector('.main-header')
const leftContainer = document.querySelector('.left-container')
const rightContainer = document.querySelector('.right-container')

// Header nav follow on scroll
const containerNav = document.querySelector(".container.last.projects")
const projectsNav = document.querySelector(".carousel-nav")

ScrollTrigger.scrollerProxy(".main-container", {
    scrollTop(value) {
        if (arguments.length) {
            bodyScrollBar.scrollTop = value
        }
        return bodyScrollBar.scrollTop
    }
})
bodyScrollBar.addListener(ScrollTrigger.update)

gsap.set(".panel", { zIndex: (i, target, targets) => targets.length - i })

const images = gsap.utils.toArray('.panel:not(.purple)');

images.forEach((image, i) => {
    const tl = gsap.timeline({
        scrollTrigger: {
            trigger: ".language",
            scroller: ".main-container",
            start: () => "top -" + (window.innerHeight * (i + 0.5)),
            end: () => "+=" + window.innerHeight,
            scrub: true,
            toggleActions: "play none reverse none",
            invalidateOnRefresh: true,
        }
    });
    if (i !== 3) tl.to(image, { opacity: 0 })
})

// Panel text
gsap.set(".panel-text", { zIndex: (i, target, targets) => targets.length - i })

// Get all text panels
var texts = gsap.utils.toArray('.panel-text')

texts.forEach((text, i) => {
    var tl = gsap.timeline({
        scrollTrigger: {
            trigger: ".language",
            scroller: ".main-container",
            start: () => "top -" + (window.innerHeight * i),
            end: () => "+=" + window.innerHeight,
            scrub: true,
            toggleActions: "play none reverse none",
            invalidateOnRefresh: true,
        }
    })

    tl.to(text, { duration: 0.33, opacity: 1, y: "0%" })

    if (i !== 3) tl.to(text, { duration: 0.33, opacity: 0, y: "-50%" }, 0.66)
})

ScrollTrigger.create({
    trigger: ".language",
    scroller: ".main-container",
    scrub: true,
    //markers: true,
    pin: true,
    start: () => "top top",
    end: () => "+=" + ((images.length + 1) * window.innerHeight),
    invalidateOnRefresh: true,
})

// Height of header
let topOffSet = 101

// Get list of lin interne
let links = gsap.utils.toArray(".button.link")

links.forEach(a => {

    let element = document.querySelector(a.getAttribute("href"))
    let linkST = ScrollTrigger.create({
        trigger: element,
        start: "top top"
    })

    a.addEventListener("click", (e) => {
        // Remove default action
        e.preventDefault()
        bodyScrollBar.scrollTo(0, linkST.start, 2000)
        goToSpecificCarousel(a.getAttribute("id"))
    })
})

// Navigation buttons

const upArrow = document.querySelector(".arrow.up")
const downArrow = document.querySelector(".arrow.down")

const sections = [...document.querySelectorAll(".container")]
const counter = document.querySelector("#page-counter")

let sectionsStart;

function sectionsStartF() {
    sectionsStart = [0]
    for (let sec = 1; sec < sections.length; sec++) {
        sectionsStart.push(ScrollTrigger.create({
            scroller: document.querySelector('.main-container'),
            trigger: sections[sec],
            start: "top top",
            invalidateOnRefresh: true
        }).start)
    }
}

sectionsStartF()

window.addEventListener('resize', () => {
    ScrollTrigger.refresh()
    sectionsStartF()
})

let options = {
    rootMargin: "0px",
    threshold: 0.30
}

const navigationTexts = [
    "Bienvenue sur mon portfolio",
    "Langages de programations",
    "Outils de dévéloppement",
    "Projets réalisés"
]

const callback = (entries) => {
    entries.forEach(entry => {
        const { target } = entry
        if (entry.intersectionRatio >= 0.30) {
            target.classList.add("is-visible")
            for (let x = 0; x < sections.length; x++) {
                if (sections[x] === target) {
                    counter.innerHTML = x + 1 + ''
                    if (x === sections.length - 1) {
                        downArrow.classList.remove('active')
                    } else {
                        downArrow.classList.add('active')
                    }
                    if (x === 0) {
                        upArrow.classList.remove('active')
                    } else {
                        upArrow.classList.add('active')
                    }

                    // Change text
                    let navigationText = document.querySelector('.welcome-text').children[0]
                    navigationText.innerHTML = navigationTexts[x]
                }
            }
        } else {
            target.classList.remove("is-visible")
        }
    })
}

const observer = new IntersectionObserver(callback, options)

sections.forEach((section) => {
    // const sectionChildren = [...section.querySelector("[data-content]").children]
    // sectionChildren.forEach((el, index) =>
    // {
    //   el.style.setProperty("--delay", `${index * 250}ms`)
    // })
    observer.observe(section)
})

downArrow.addEventListener('click', (e) => {
    // Remove default action
    e.preventDefault()

    // Check if he has active class
    if (!downArrow.classList.contains("active")) {
        return
    }

    let count = parseInt(counter.innerHTML)

    if (count !== sections.length) {
        // Scroll to the section
        bodyScrollBar.scrollTo(0, sectionsStart[count], 1000)
    } else {
        // Else remove the class active on down arrow
        downArrow.classList.remove("active")
    }
})

upArrow.addEventListener('click', (e) => {
    // Remove default action
    e.preventDefault()

    // Check if he has active class
    if (!upArrow.classList.contains("active")) {
        return
    }

    let count = parseInt(counter.innerHTML) - 2

    if (count >= 0) {
        // Scroll to the section
        bodyScrollBar.scrollTo(0, sectionsStart[count], 1000)
    } else {
        // Else remove the class active on down arrow
        upArrow.classList.remove("active")
    }
})

// Carousel
let carouselIndex = 0;

const carouselLeftController = document.querySelector(".arrow.left.carousel-button")
const carouselRightController = document.querySelector(".arrow.right.carousel-button")

carouselLeftController.addEventListener('click', () => {
    carouselIndex -= 1
    nextCarousel()
})

carouselRightController.addEventListener('click', () => {
    carouselIndex += 1
    nextCarousel()
})

const languages = [
    "python",
    "java",
    "web"
]

const slides = document.querySelectorAll(".carousel-item")
const languageSlides = [...document.querySelector(".carousel-language").children]
const textSlides = [...document.querySelector(".language-text").children[0].children]

function removeDisplay() {
    for (let x = 0; x < slides.length; x++) {
        slides[x].style.display = "none"
        slides[x].style.opacity = 0
        languageSlides[x].style.display = "none"
        textSlides[x].style.display = "none"
    }

    languageSlides[carouselIndex].style.display = "flex"
    textSlides[carouselIndex].style.display = "flex"

    setTimeout(() => {
        slides[carouselIndex].style.display = "flex"
        setTimeout(() => {
            slides[carouselIndex].style.opacity = 1
        }, 100)
    }, 100)
}

function goToSpecificCarousel(language) {
    for (let i = 0; i < languages.length; i++){
        if (languages[i] === language) {
            carouselIndex = i;
            break;
        }
    }

    if (slides[carouselIndex].style.display !== "flex") {
        removeDisplay();
    }
}

function nextCarousel(first = false) {

    if (carouselIndex >= slides.length) {
        carouselIndex = 0
    } else if (carouselIndex < 0) {
        carouselIndex = slides.length - 1
    }

    removeDisplay();


    if (!first) {
        bodyScrollBar.scrollTo(0, sectionsStart[3], 100)
    }
}

nextCarousel(true);


// Button link
const linkButtons = document.querySelectorAll('.button.link');

linkButtons.forEach(button => {
    ["mouseenter", "mouseout", "touchstart"].forEach(evt => {
        button.addEventListener(evt, e => {
            let parentOffset = button.getBoundingClientRect(),
                relX = e.pageX - parentOffset.left,
                relY = e.pageY - parentOffset.top;

            const span = button.getElementsByTagName("span");

            span[0].style.top = relY + "px";
            span[0].style.left = relX + "px";
        })
    })
})

// Menu burger
const menuBurgerButton = document.querySelector('.menu-button')
const menuBurger = document.querySelector('.menu')

menuBurgerButton.onclick = function (e) {
    e.preventDefault()
    menuBurger.classList.toggle('active')
    setTimeout(() => {
        if (menuBurger.classList.contains('active')) {
            menuBurger.style.opacity = 1
        }
        else {
            menuBurger.style.opacity = 0
        }
    }, 100)
}

// Button menu
const buttonMenu = document.querySelectorAll('.menu-link')

buttonMenu.forEach((button, index) => {
    button.addEventListener('click', (e) => {
        e.preventDefault()
        bodyScrollBar.scrollTo(0, sectionsStart[index])
        menuBurger.classList.toggle('active')
    })
})

bodyScrollBar.addListener(({ offset }) => {
    mainHeader.style.top = offset.y + 'px'
    leftContainer.style.top = 101 + offset.y + 'px'
    rightContainer.style.top = 101 + offset.y + 'px'
    menuBurger.style.top = offset.y + 'px'
    projectsNav.style.top  = offset.y + 'px'
})

// Text Sphere
const myTags = [
    'JavaScript', 'CSS', 'HTML',
    'JavaFX', 'PHP', 'CSharp (Unity)',
    'Java', 'Python', 'Git', 'Symfony',
    'Laravel',
]

const tagCloud = TagCloud('.sphere-text', myTags, {
    // radius in px
    radius: 250,

    // animation speed
    // slow, normal, fast
    maxSpeed: 'slow',
    initSpeed: 'slow',

    // 0 = top
    // 90 = left
    // 135 = right-bottom
    direction: 135,

    // interact with cursor move on mouse out
    keep: true
})
