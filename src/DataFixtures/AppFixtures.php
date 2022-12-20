<?php

namespace App\DataFixtures;

use App\Entity\Category;
use App\Factory\ProjectFactory;
use Doctrine\Bundle\FixturesBundle\Fixture;
use Doctrine\Persistence\ObjectManager;
use Exception;

class AppFixtures extends Fixture
{
    /**
     * @throws Exception
     */
    public function load(ObjectManager $manager): void
    {
        // $product = new Product();
        // $manager->persist($product);

        $categoryOne = new Category();
        $categoryTwo = new Category();
        $categoryThree = new Category();
        $categoryFour = new Category();
        $categoryFive = new Category();
        $categorySix = new Category();

        $categoryOne -> setName("html");
        $categoryTwo -> setName("css");
        $categoryThree -> setName("js");
        $categoryFour -> setName("php");
        $categoryFive -> setName("python");
        $categorySix -> setName("java");

        $manager -> persist($categoryOne);
        $manager -> persist($categoryTwo);
        $manager -> persist($categoryThree);
        $manager -> persist($categoryFour);
        $manager -> persist($categoryFive);
        $manager -> persist($categorySix);

        for ($i = 0; $i < 20; $i++) {
            $p = ProjectFactory::createOne();

            $random = random_int(0, 5);
            if ($random == 0) {
                $p -> addCategory($categoryOne);
            } else if ($random == 1) {
                $p -> addCategory($categoryTwo);
            } else if ($random == 2) {
                $p -> addCategory($categoryThree);
            } else if ($random == 3) {
                $p -> addCategory($categoryFour);
            } else if ($random == 4) {
                $p -> addCategory($categoryFive);
            } else if ($random == 5) {
                $p -> addCategory($categorySix);
            }

            $p -> setName('Project '.$i);
        }

        $manager->flush();
    }
}
