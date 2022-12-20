<?php

namespace App\Controller;

use App\Repository\CategoryRepository;
use App\Repository\ProjectRepository;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;

class HomeController extends AbstractController
{
    #[Route('/', name: 'app_home')]
    public function index(ProjectRepository $projectRepository, CategoryRepository $categoryRepository): Response
    {
        $projects = array();

        $projects["first"] = $projectRepository -> findThreeFirstProject();
        $projects["python"] = $projectRepository -> findAllJoinedToOneCategory($categoryRepository -> findIdByName("python"));
        $projects["java"] = $projectRepository -> findAllJoinedToOneCategory($categoryRepository -> findIdByName("java"));
        $projects["php"] = $projectRepository -> findAllJoinedToOneCategory($categoryRepository -> findIdByName("php"));

        return $this->render('home/index.html.twig', [
            'projects' => $projects
        ]);
    }
}
