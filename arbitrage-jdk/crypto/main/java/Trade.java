/**
 * @ClassName Trade
 * @Description Trading behavior abstraction
 * @Author happyboy
 * @Date 2025/9/9 2:18
 * @Version 1.0
 */
public class Trade {
    private Arbitrage arbitrage;

    public Arbitrage getArbitrage() {
        return arbitrage;
    }

    public void setArbitrage(Arbitrage arbitrage) {
        this.arbitrage = arbitrage;
    }
}
