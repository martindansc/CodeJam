#include <iostream>
#include <vector>
#include <string>
#include <math.h>

#define M_PI 3.14159265358979323846

using namespace std;

struct Point
{
    Point()
    {
    }

    Point(double x, double y)
    {
        this->x = x;
        this->y = y;
    }

    double get_distance_to(Point to)
    {
        double square_dist = pow(to.x - x, 2) + pow(to.y - y, 2);
        return sqrt(square_dist);
    }

    double x, y;
};

struct Triangle
{
    Triangle() {}

    Triangle(double h, double l, bool is_h)
    {
        double m = sqrt(pow(h, 2) - pow(l, 2));
        this->p1 = Point(0, 0);

        if (is_h)
        {
            this->p2 = Point(m, 0);
            this->p3 = Point(m, l);
        }
        else
        {
            this->p2 = Point(l, 0);
            this->p3 = Point(l, m);
        }
    }

    Triangle(Point p1, Point p2, Point p3) : p1(p1), p2(p2), p3(p3)
    {
        if (p2.x < p1.x || p2.y < p1.y)
        {
            cerr << "ERROR Triangle malformed";
            exit(1);
        }

        if (p3.x < p1.x || p3.y < p1.y)
        {
            cerr << "ERROR Triangle malformed";
            exit(1);
        }
    }

    double get_area()
    {
        double a = p1.get_distance_to(p2);
        double b = p2.get_distance_to(p3);
        double c = p3.get_distance_to(p1);

        double area = a * c * sin(this->get_alpha()) / 2;

        return area;
    }

    double get_alpha()
    {

        double a = p1.get_distance_to(p2);
        double b = p3.get_distance_to(p1);
        double c = p2.get_distance_to(p3);

        return acos((pow(c, 2) - pow(a, 2) - pow(b, 2)) / (-2 * a * b));
    }

    Point p1, p2, p3;
};

struct Square
{
    Square(Point left_down_point, double g) : g(g)
    {
        this->p1 = left_down_point;
        this->p2 = Point(left_down_point.x + g, left_down_point.y);
        this->p3 = Point(left_down_point.x + g, left_down_point.y + g);
        this->p4 = Point(left_down_point.x, left_down_point.y + g);
    }

    double check_if_out_of_cercle(double radius)
    {
        return radius * radius < p1.x * p1.x + p1.y * p1.y;
    }

    double check_if_intersects_cercle(double radius)
    {
        return radius * radius < p3.x * p3.x + p3.y * p3.y && radius * radius > p1.x * p1.x + p1.y * p1.y;
    }

    double get_area()
    {
        return g * g;
    }

    vector<Point> get_intersection_with_cercle(double R)
    {
        vector<Point> points = vector<Point>();
        Triangle t1 = Triangle(R, p1.y, true);
        if (!isnan(t1.p3.x) && t1.p3.x >= p1.x && t1.p3.x <= p2.x)
        {
            points.push_back(t1.p3);
        }

        Triangle t2 = Triangle(R, p4.y, true);
        if (!isnan(t2.p3.x) && t2.p3.x >= p4.x && t2.p3.x <= p3.x)
        {
            points.push_back(t2.p3);
        }

        Triangle t3 = Triangle(R, p1.x, false);
        if (!isnan(t3.p3.y) && t3.p3.y >= p1.y && t3.p3.y <= p4.y)
        {
            points.push_back(t3.p3);
        }

        Triangle t4 = Triangle(R, p2.x, false);
        if (!isnan(t4.p3.y) && t4.p3.y >= p2.y && t4.p3.y <= p3.y)
        {
            points.push_back(t4.p3);
        }

        return points;
    }

    Point p1, p2, p3, p4;
    double g;
};

double get_area_cercle(double radius)
{
    return M_PI * pow(radius, 2);
}

int get_number_of_squares_in_line(double R, double g, double r)
{
    double big_square_g = g + 2 * r;
    return ceil(R / big_square_g);
}

double check_if_out_of_cercle(double radius, Point p1)
{
    return radius * radius < p1.x * p1.x + p1.y * p1.y;
}

void solve_instance(int problem_instance)
{
    double f, R, t, r, g;
    cin >> f >> R >> t >> r >> g;

    t = t + f;
    g = g - 2 * f;
    r = r + f;

    double inner_radius = R - t;
    double big_square_g = g + 2 * r;

    double outer_area = get_area_cercle(R);
    double inner_area = get_area_cercle(inner_radius);

    int num_completed = get_number_of_squares_in_line(R, g, r);
    double sum_squares_area = 0;
    for (int i = 0; i < num_completed; i++)
    {
        for (int j = 0; j < num_completed; j++)
        {
            Square s = Square(Point(i * big_square_g + r, j * big_square_g + r), g);
            if (s.check_if_intersects_cercle(inner_radius))
            {
                vector<Point> i_points = s.get_intersection_with_cercle(inner_radius);
                if (i_points.size() != 2)
                {
                    cerr << "ERROR: Intersection has more or less than 2 points! " << i_points.size() << endl;
                    vector<Point> i_points = s.get_intersection_with_cercle(inner_radius);
                    exit(1);
                }

                Triangle inner_triangle = Triangle(s.p1, i_points[0], i_points[1]);
                sum_squares_area += inner_triangle.get_area();

                if (!check_if_out_of_cercle(inner_radius, s.p2))
                {
                    Triangle inner_triangle_2;
                    if (i_points[0].x == s.p2.x || i_points[0].y == s.p2.y)
                    {
                        inner_triangle_2 = Triangle(s.p1, s.p2, i_points[0]);
                    }
                    else
                    {
                        inner_triangle_2 = Triangle(s.p1, s.p2, i_points[1]);
                    }

                    sum_squares_area += inner_triangle_2.get_area();
                }

                if (!check_if_out_of_cercle(inner_radius, s.p4))
                {
                    Triangle inner_triangle_4;
                    if (i_points[0].x == s.p4.x || i_points[0].y == s.p4.y)
                    {
                        inner_triangle_4 = Triangle(s.p1, i_points[0], s.p4);
                    }
                    else
                    {
                        inner_triangle_4 = Triangle(s.p1, i_points[1], s.p4);
                    }
                    sum_squares_area += inner_triangle_4.get_area();
                }

                Triangle cone_triangle = Triangle(Point(0, 0), i_points[0], i_points[1]);
                sum_squares_area += (cone_triangle.get_alpha() * inner_area / (2 * M_PI)) - cone_triangle.get_area();
            }
            else if (!s.check_if_out_of_cercle(inner_radius))
            {
                sum_squares_area += s.get_area();
            }
        }
    }

    double prob = (outer_area - sum_squares_area * 4) / outer_area;

    if (prob < 0)
        prob = 0;

    cout.precision(6);
    cout << "Case #" << fixed << problem_instance << ": " << prob << endl;
}

int main()
{

    int number_of_problems;
    cin >> number_of_problems;
    for (int i = 0; i < number_of_problems; i++)
    {
        solve_instance(i + 1);
    }
}